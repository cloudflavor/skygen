#!/usr/bin/env python3
import argparse
import os
import re
import sys
import time
from collections import deque
from urllib.parse import urljoin, urlparse, urldefrag

import requests
from bs4 import BeautifulSoup

BASE = "https://www.scaleway.com"
START = "https://www.scaleway.com/en/developers/api/"
SCOPE_PREFIX = "/en/developers/api/"
STATIC_PREFIX = "/en/developers/static/"
YML_RE = re.compile(r"\.ya?ml($|\?)", re.IGNORECASE)


def norm(url: str) -> str:
    url, _ = urldefrag(url)
    return url


def in_scope(url: str) -> bool:
    p = urlparse(url)
    return (
        p.scheme in ("http", "https")
        and p.netloc == urlparse(BASE).netloc
        and p.path.startswith(SCOPE_PREFIX)
    )


def is_yaml_url(url: str) -> bool:
    p = urlparse(url)
    return p.netloc == urlparse(BASE).netloc and (
        p.path.startswith(STATIC_PREFIX) and YML_RE.search(p.path)
    )


def fetch(session: requests.Session, url: str) -> requests.Response:
    r = session.get(url, timeout=30)
    r.raise_for_status()
    return r


def main() -> int:
    ap = argparse.ArgumentParser()
    ap.add_argument("--out", default="scaleway-openapi", help="output directory")
    ap.add_argument(
        "--sleep", type=float, default=0.15, help="sleep between requests (seconds)"
    )
    ap.add_argument(
        "--max-pages", type=int, default=1000, help="max HTML pages to crawl"
    )
    args = ap.parse_args()

    os.makedirs(args.out, exist_ok=True)

    session = requests.Session()
    session.headers.update({"User-Agent": "scaleway-openapi-fetch/1.0"})

    # Crawl all /en/developers/api/* pages
    q = deque([START])
    seen = set()
    api_pages = []

    while q and len(api_pages) < args.max_pages:
        url = norm(q.popleft())
        if url in seen:
            continue
        if url != START and not in_scope(url):
            continue

        try:
            r = fetch(session, url)
        except Exception as e:
            print(f"[warn] page fetch failed: {url}: {e}", file=sys.stderr)
            continue

        seen.add(url)
        ctype = r.headers.get("content-type", "")
        if "text/html" not in ctype:
            time.sleep(args.sleep)
            continue

        api_pages.append(url)
        soup = BeautifulSoup(r.text, "html.parser")

        for a in soup.find_all("a", href=True):
            nxt = norm(urljoin(url, a["href"].strip()))
            if in_scope(nxt) and nxt not in seen:
                q.append(nxt)

        time.sleep(args.sleep)

    # From each API page, extract the real download hrefs (YAML)
    found = {}  # yaml_url -> source_page
    for page in api_pages:
        try:
            r = fetch(session, page)
        except Exception as e:
            print(f"[warn] page refetch failed: {page}: {e}", file=sys.stderr)
            continue

        soup = BeautifulSoup(r.text, "html.parser")
        for a in soup.find_all("a", href=True):
            href = a["href"].strip()
            full = norm(urljoin(page, href))
            if is_yaml_url(full):
                found.setdefault(full, page)

        time.sleep(args.sleep)

    # Write manifest of ACTUAL download URIs
    manifest = os.path.join(args.out, "manifest.tsv")
    with open(manifest, "w", encoding="utf-8") as f:
        f.write("yaml_url\tsource_page\n")
        for yaml_url, source_page in sorted(found.items()):
            f.write(f"{yaml_url}\t{source_page}\n")

    # Download YAMLs (filename from URL path)
    for yaml_url in sorted(found.keys()):
        name = os.path.basename(urlparse(yaml_url).path)
        out_path = os.path.join(args.out, name)
        try:
            r = fetch(session, yaml_url)
            with open(out_path, "wb") as f:
                f.write(r.content)
            print(yaml_url)
        except Exception as e:
            print(f"[warn] yaml fetch failed: {yaml_url}: {e}", file=sys.stderr)
        time.sleep(args.sleep)

    print(f"\nSaved manifest: {manifest}", file=sys.stderr)
    return 0


if __name__ == "__main__":
    raise SystemExit(main())

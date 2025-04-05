import requests
from typing import Dict
from pydantic import BaseModel
from dataclasses import dataclass


class AnswerEntry(BaseModel):
    discord: str
    license: str
    license2: str
    xbl: str
    fivem: str
    steam: str
    live: str
    pseudo: str
    date: str


class SearchResult(BaseModel):
    time: int
    value: int
    answer: Dict[str, AnswerEntry]


def search_query(query: str, api_key: str) -> SearchResult:
    url = "https://api.searchhub.ru/search"
    return search_query_with_url(query, api_key, url)


def search_query_with_url(query: str, api_key: str, url: str) -> SearchResult:
    headers = {
        "Content-Type": "application/json",
        "x-api-key": api_key
    }

    payload = {
        "query": query
    }

    response = requests.post(url, json=payload, headers=headers)
    response.raise_for_status()

    data = response.json()
    return SearchResult(**data)

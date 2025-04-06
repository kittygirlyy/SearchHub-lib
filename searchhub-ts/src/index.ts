import axios from "axios";

export interface AnswerEntry {
  discord: string;
  license: string;
  license2: string;
  xbl: string;
  fivem: string;
  steam: string;
  live: string;
  pseudo: string;
  date: string;
}

export interface SearchResult {
  time: number;
  value: number;
  answer: Record<string, AnswerEntry>;
}

export async function searchQuery(query: string, apiKey: string): Promise<SearchResult> {
  const url = "https://api.searchhub.ru/search";
  return await searchQueryWithUrl(query, apiKey, url);
}

export async function searchQueryWithUrl(query: string, apiKey: string, url: string): Promise<SearchResult> {
  const headers = {
    "Content-Type": "application/json",
    "x-api-key": apiKey,
  };

  const payload = {
    query,
  };

  const response = await axios.post(url, payload, { headers });
  return response.data as SearchResult;
}

import type {
  GameDNA,
  ValidationResult,
  LockedGameDNA,
  VersionInfo,
  ListGameDNARequest,
} from '@entropic/types';
import type { RequestInterceptor } from './interceptors.js';

export interface GameDNAServiceClientOptions {
  baseUrl: string;
  interceptors?: RequestInterceptor[];
}

export class GameDNAServiceClient {
  private baseUrl: string;
  private interceptors: RequestInterceptor[];

  constructor(options: GameDNAServiceClientOptions) {
    this.baseUrl = options.baseUrl.replace(/\/$/, '');
    this.interceptors = options.interceptors ?? [];
  }

  private async request<T>(path: string, init: RequestInit = {}): Promise<T> {
    let url: RequestInfo = `${this.baseUrl}${path}`;
    let finalInit: RequestInit = {
      ...init,
      headers: {
        'Content-Type': 'application/json',
        ...init.headers,
      },
    };

    // Apply request interceptors
    for (const interceptor of this.interceptors) {
      if (interceptor.onRequest) {
        [url, finalInit] = await interceptor.onRequest(url, finalInit);
      }
    }

    let response = await fetch(url, finalInit);

    // Apply response interceptors
    for (const interceptor of this.interceptors) {
      if (interceptor.onResponse) {
        response = await interceptor.onResponse(response);
      }
    }

    if (!response.ok) {
      throw new Error(`API request failed: ${response.statusText}`);
    }

    return response.json();
  }

  async createGameDNA(config: GameDNA): Promise<GameDNA> {
    return this.request<GameDNA>('/api/v1/dna', {
      method: 'POST',
      body: JSON.stringify({ config }),
    });
  }

  async getGameDNA(id: string): Promise<GameDNA> {
    return this.request<GameDNA>(`/api/v1/dna/${id}`, {
      method: 'GET',
    });
  }

  async listGameDNA(filters?: ListGameDNARequest['filters']): Promise<GameDNA[]> {
    const params = new URLSearchParams();
    if (filters?.genre) params.set('genre', String(filters.genre));
    if (filters?.platform) params.set('platform', String(filters.platform));
    if (filters?.search) params.set('search', filters.search);

    const query = params.toString();
    const path = query ? `/api/v1/dna?${query}` : '/api/v1/dna';

    const response = await this.request<{ configs: GameDNA[] }>(path, {
      method: 'GET',
    });

    return response.configs;
  }

  async updateGameDNA(id: string, config: GameDNA): Promise<GameDNA> {
    return this.request<GameDNA>(`/api/v1/dna/${id}`, {
      method: 'PUT',
      body: JSON.stringify({ config }),
    });
  }

  async validateGameDNA(config: GameDNA): Promise<ValidationResult> {
    const response = await this.request<{ result: ValidationResult }>('/api/v1/dna/validate', {
      method: 'POST',
      body: JSON.stringify({ config }),
    });
    return response.result;
  }

  async publishGameDNA(id: string): Promise<LockedGameDNA> {
    const response = await this.request<{ locked_config: LockedGameDNA }>(`/api/v1/dna/${id}/publish`, {
      method: 'POST',
    });
    return response.locked_config;
  }

  async getVersionHistory(id: string): Promise<VersionInfo[]> {
    const response = await this.request<{ versions: VersionInfo[] }>(`/api/v1/dna/${id}/versions`, {
      method: 'GET',
    });
    return response.versions;
  }

  async rollbackToVersion(id: string, version: number): Promise<GameDNA> {
    return this.request<GameDNA>(`/api/v1/dna/${id}/rollback/${version}`, {
      method: 'POST',
    });
  }
}

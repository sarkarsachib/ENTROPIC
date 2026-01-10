export interface RequestInterceptor {
  onRequest?: (input: RequestInfo, init: RequestInit) => Promise<[RequestInfo, RequestInit]> | [RequestInfo, RequestInit];
  onResponse?: (response: Response) => Promise<Response> | Response;
}

export function createAuthInterceptor(getToken: () => string | undefined): RequestInterceptor {
  return {
    onRequest: (input, init) => {
      const token = getToken();
      if (!token) return [input, init];

      const headers = new Headers(init.headers);
      headers.set('Authorization', `Bearer ${token}`);
      return [input, { ...init, headers }];
    },
  };
}

export function createLoggingInterceptor(logger: (msg: string) => void): RequestInterceptor {
  return {
    onRequest: (input, init) => {
      logger(`→ ${String(input)} ${init.method ?? 'GET'}`);
      return [input, init];
    },
    onResponse: async (res) => {
      logger(`← ${res.status} ${res.url}`);
      return res;
    },
  };
}

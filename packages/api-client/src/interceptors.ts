export interface RequestInterceptor {
  onRequest?: (input: RequestInfo, init: RequestInit) => Promise<[RequestInfo, RequestInit]> | [RequestInfo, RequestInit];
  onResponse?: (response: Response) => Promise<Response> | Response;
}

/**
 * Produces a RequestInterceptor that injects a Bearer `Authorization` header when a token is available.
 *
 * @param getToken - Function that returns the current auth token or `undefined` if none is available
 * @returns A RequestInterceptor whose `onRequest` adds `Authorization: Bearer <token>` to the request headers when `getToken()` returns a token; otherwise the request is left unchanged
 */
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

/**
 * Creates an interceptor that logs outgoing requests and incoming responses.
 *
 * @param logger - Function called with a single message string for each logged event. Request messages are formatted as `→ <input> <method>` (method defaults to `GET` if unset); response messages are formatted as `← <status> <url>`.
 * @returns A RequestInterceptor whose `onRequest` logs the request and returns it unchanged, and whose `onResponse` logs the response status and URL and returns the response unchanged.
 */
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
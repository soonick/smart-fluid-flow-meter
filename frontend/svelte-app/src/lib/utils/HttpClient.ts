import { getCookie } from './Cookies';

import { AuthorizationCookie, BackendUrl } from './Constants';

function getToken(): string {
  return getCookie(AuthorizationCookie);
}

export async function httpGet(path: string) {
  const requestHeaders: HeadersInit = new Headers();
  requestHeaders.set('Accept', 'application/json');
  requestHeaders.set('Authorization', getToken());

  return fetch(BackendUrl + path, {
    method: 'GET',
    headers: requestHeaders
  });
}

export async function httpPost(path: string, data: object) {
  const requestHeaders: HeadersInit = new Headers();
  requestHeaders.set('Accept', 'application/json');
  requestHeaders.set('Content-Type', 'application/json');
  requestHeaders.set('Authorization', getToken());

  return fetch(BackendUrl + path, {
    method: 'POST',
    headers: requestHeaders,
    body: JSON.stringify(data)
  });
}

export async function httpPut(path: string, data: object) {
  const requestHeaders: HeadersInit = new Headers();
  requestHeaders.set('Accept', 'application/json');
  requestHeaders.set('Content-Type', 'application/json');
  requestHeaders.set('Authorization', getToken());

  return fetch(BackendUrl + path, {
    method: 'PUT',
    headers: requestHeaders,
    body: JSON.stringify(data)
  });
}

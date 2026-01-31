import http from 'k6/http';
import encoding from 'k6/encoding';
import { check } from 'k6';

export const options = {
  scenarios: {
    run_auth: {
      exec: 'run_auth',
      executor: 'ramping-arrival-rate',
      stages: [
        { target: 100000, duration: '20s' },
        { target: 1000000, duration: '20s' },
        { target: 10000000, duration: '20s' },
      ],
      startRate: 100000,
      timeUnit: '20s',
      preAllocatedVUs: 1000,
      maxVUs: 5000,
    }
  }
};

export function run_auth() {
  const user = 'jennifer.davis@hotmail.com';
  const pass = 'YmPg2VP3h2kk';
  const credentials = `${user}:${pass}`;
  const encodedCredentials = encoding.b64encode(credentials);
  const params = {
    headers: {
      Authorization: `Basic ${encodedCredentials}`,
    },
  };
  let url = `${__ENV.API_BASE_URL}/api/auth`;
  let res = http.post(url, null, params);
  let key = `${url}: 200`;
  check(res, { [key]: (r) => r.status === 200  });
}

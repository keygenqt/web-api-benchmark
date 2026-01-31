import http from 'k6/http';
import encoding from 'k6/encoding';
import { check } from 'k6';

export const options = {
  scenarios: {
    run_user: {
      exec: 'run_user',
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

export function run_user() {
  let url = `${__ENV.API_BASE_URL}/api/users/1`;
  let res = http.get(url);
  let key = `${url}: 403`;
  check(res, { [key]: (r) => r.status === 403  });
}

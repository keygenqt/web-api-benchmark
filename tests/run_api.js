import http from 'k6/http';
import { check } from 'k6';

export const options = {
  scenarios: {
    run_api: {
      exec: 'run_api',
      executor: 'ramping-arrival-rate',
      stages: [
        { target: 100000, duration: '20s' },
        { target: 500000, duration: '20s' },
        { target: 1000000, duration: '20s' },
      ],
      startRate: 100000,
      timeUnit: '5s',
      preAllocatedVUs: 1000,
      maxVUs: 5000,
    }
  }
};

export function run_api() {
  let url = `${__ENV.API_BASE_URL}/api`;
  let res = http.get(url);
  let key = `${url}: 404`;
  check(res, { [key]: (r) => r.status === 404  });
}

import http from 'k6/http';
import { check } from 'k6';

export const options = {
  scenarios: {
    run_home: {
      exec: 'run_home',
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

export function run_home() {
  let url = `${__ENV.API_BASE_URL}`;
  let res = http.get(url);
  let key = `${url}: 200`;
  check(res, { [key]: (r) => r.status === 200  });
}

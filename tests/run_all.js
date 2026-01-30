// run_api:
// - handles route
// - serialization
import { options as options_run_api } from './run_api.js';
export { run_api } from './run_api.js';

// run_auth:
// - db query
// - serialization
// - validate argon2
// - encode and set cookies
import { options as options_run_auth } from './run_auth.js';
export { run_auth } from './run_auth.js';

// run_home:
// - html output
import { options as options_run_home } from './run_home.js';
export { run_home } from './run_home.js';

// run_user:
// - check access
import { options as options_run_user } from './run_user.js';
export { run_user } from './run_user.js';

// common options
export const options = {
  scenarios: {
    ...options_run_api.scenarios,
    ...options_run_auth.scenarios,
    ...options_run_home.scenarios,
    ...options_run_user.scenarios,
  }
}

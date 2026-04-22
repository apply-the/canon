# Verification Brief: E2E Test Flakiness 

## Claims Under Test
- Claim 1: The Playwright checkout flow fails nondeterministically (30% failure rate) solely due to third-party Stripe sandbox latency, not our internal state transitions.
- Claim 2: The frontend correctly buffers and retries failed payment intents up to 3 times before displaying the "Payment Failed" modal.

## Evidence Basis
- CI Logs from GitHub Actions (Job IDs: 1044421, 1044521, 1044621).
- `e2e/tests/checkout.spec.ts`
- Error response trace from the API gateway (`logs/gateway-prod.log`).

## Contract Surface
- The interface for Stripe's `createPaymentIntent` and `confirmCardPayment`. 
- The frontend UI state machine which dictates exactly when the loading spinner must yield to an error component.
- The `stripe-mock` mock endpoint behaviour in `tests/mocks/`.

## Risk Boundary
- If the true cause of the flakiness is our Next.js frontend state wiping local session state on a network reset, rather than Stripe itself, we are masking a critical conversion-losing bug in production.

## Challenge Focus
- Can we prove the Playwright network stubbing strictly enforces the 3-retry UI behavior under an artificial 2000ms delay and 500 error?
- Analyze the `e2e/tests/checkout.spec.ts` script for any implicit `await page.waitForTimeout()` lines that mask true race conditions.

## Out of Scope
- Evaluating general backend API latency or database performance. 
- Fixing the bug (we only want to verify the claim of its origin).
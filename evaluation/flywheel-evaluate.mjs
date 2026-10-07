import { spawnSync } from "node:child_process";

const modulePath = process.env.FLYWHEEL_MODULE
  ?? "/home/ruv/.local/share/dream-machine/evaluators/node_modules/@metaharness/flywheel/dist/index.js";
const { makeSigner, runFlywheelGenerations, verifyReplayBundle } = await import(modulePath);

const candidate = process.env.TOOLRECALL_CANDIDATE ?? "target/debug/toolrecall-candidate";
const evaluate = async (policy, suite) => {
  const input = JSON.stringify({
    variantId: `${suite.id}-${policy.top_k}-${policy.max_loaded}`,
    genome: { top_k: Number(policy.top_k), max_loaded: Number(policy.max_loaded) },
  });
  const run = spawnSync(candidate, { input, encoding: "utf8", timeout: 30_000 });
  if (run.status !== 0) throw new Error(run.stderr || `candidate exit ${run.status}`);
  return JSON.parse(run.stdout);
};

const proposer = async (base, target) => {
  if (target === "max_loaded") return String(Math.max(2, Number(base.policy[target]) - 1));
  return "1";
};

const result = await runFlywheelGenerations({
  rootPolicy: { top_k: "1", max_loaded: "3" },
  proposer,
  evaluator: evaluate,
  holdout: { id: "toolrecall-frozen-v1", items: ["q1", "q2", "q3", "q4", "q5", "q6"] },
  anchor: { id: "toolrecall-anchor-v1", items: ["q1", "q2", "q3", "q4", "q5", "q6"] },
  mutationTargets: ["top_k", "max_loaded"],
  maxGenerations: 2,
  signer: makeSigner(),
  now: (generation) => `2026-10-05-generation-${generation}`,
  cacheEvaluations: true,
  dataSource: "SYNTHETIC_FROZEN_V1",
});
const replay = verifyReplayBundle(result.replayBundle);
if (!replay.pass) throw new Error(`replay failed: ${JSON.stringify(replay.checks)}`);
console.log(JSON.stringify({
  generations: result.generationsRun,
  promotions: result.promotions.length,
  milestoneReached: result.milestoneReached,
  finalPolicy: result.finalPolicy,
  replay,
  authority: "none",
}, null, 2));

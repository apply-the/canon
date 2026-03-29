param(
  [string]$Profile = "",
  [string]$RunId = "",
  [string]$Target = ""
)

Write-Output "Next:"
switch ($Profile) {
  "run-started" {
    Write-Output "- Use `$canon-status for run $RunId."
    Write-Output "- Use `$canon-inspect-invocations for request-level decisions."
    Write-Output "- Use `$canon-inspect-evidence for evidence lineage."
  }
  "status-completed" {
    Write-Output "- Use `$canon-inspect-invocations for request-level decisions on run $RunId."
    Write-Output "- Use `$canon-inspect-evidence for evidence lineage."
    Write-Output "- Use `$canon-inspect-artifacts if you need emitted file paths."
  }
  "status-gated" {
    Write-Output "- Use `$canon-approve for target $Target on run $RunId."
    Write-Output "- Use `$canon-status after approval, or `$canon-resume if Canon still requires continuation."
  }
  "inspect" {
    Write-Output "- Use `$canon-status for the latest state of run $RunId."
    Write-Output "- Use `$canon-inspect-artifacts if you need emitted file paths."
  }
  "approval-recorded" {
    Write-Output "- Use `$canon-status to confirm the post-approval run state."
    Write-Output "- Use `$canon-resume for run $RunId only if Canon still requires continuation."
  }
  "gated" {
    Write-Output "- Use `$canon-approve for target $Target on run $RunId."
    Write-Output "- Use `$canon-status after approval, or `$canon-resume if Canon still requires continuation."
  }
  "resumed" {
    Write-Output "- Use `$canon-status for the resumed run $RunId."
    Write-Output "- Use `$canon-inspect-evidence to review the updated evidence bundle."
  }
  "inspect-artifacts" {
    Write-Output "- Use `$canon-inspect-evidence for linked runtime evidence."
    Write-Output "- Use `$canon-status for the latest state of run $RunId."
  }
  default {
    Write-Output "- Use `$canon-status to inspect the current Canon run."
  }
}

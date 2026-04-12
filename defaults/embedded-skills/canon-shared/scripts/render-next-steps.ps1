param(
  [string]$Profile = "",
  [string]$RunId = "",
  [string]$Target = ""
)

function Write-NextSteps {
  param(
    [string]$Recommended,
    [string[]]$PossibleActions = @()
  )

  Write-Output "Recommended Next Step:"
  Write-Output "- $Recommended"

  if ($PossibleActions.Count -gt 0) {
    Write-Output ""
    Write-Output "Possible Actions:"
    foreach ($Action in $PossibleActions) {
      Write-Output "- $Action"
    }
  }
}

switch ($Profile) {
  "run-started" {
    Write-NextSteps -Recommended "Use `$canon-status for run $RunId." -PossibleActions @(
      "Use `$canon-inspect-invocations for request-level decisions.",
      "Use `$canon-inspect-evidence for evidence lineage."
    )
  }
  "status-completed" {
    Write-NextSteps -Recommended "Use `$canon-inspect-evidence for evidence lineage on run $RunId." -PossibleActions @(
      "Use `$canon-inspect-invocations for request-level decisions on run $RunId.",
      "Use `$canon-inspect-artifacts if you need emitted file paths."
    )
  }
  "status-gated" {
    Write-NextSteps -Recommended "Use `$canon-inspect-evidence for run $RunId before recording approval." -PossibleActions @(
      "Use `$canon-approve for target $Target on run $RunId after review.",
      "Use `$canon-status after approval, or `$canon-resume if Canon still requires continuation."
    )
  }
  "inspect" {
    Write-NextSteps -Recommended "Use `$canon-status for the latest state of run $RunId." -PossibleActions @(
      "Use `$canon-inspect-artifacts if you need emitted file paths."
    )
  }
  "approval-recorded" {
    Write-NextSteps -Recommended "Use `$canon-resume for run $RunId only if Canon still requires continuation." -PossibleActions @(
      "Use `$canon-status to confirm the post-approval run state."
    )
  }
  "gated" {
    Write-NextSteps -Recommended "Use `$canon-inspect-evidence for run $RunId before recording approval." -PossibleActions @(
      "Use `$canon-approve for target $Target on run $RunId after review.",
      "Use `$canon-status after approval, or `$canon-resume if Canon still requires continuation."
    )
  }
  "resumed" {
    Write-NextSteps -Recommended "Use `$canon-inspect-evidence to review the updated evidence bundle for run $RunId." -PossibleActions @(
      "Use `$canon-status for the resumed run $RunId."
    )
  }
  "inspect-artifacts" {
    Write-NextSteps -Recommended "Use `$canon-inspect-evidence for linked runtime evidence on run $RunId." -PossibleActions @(
      "Use `$canon-status for the latest state of run $RunId."
    )
  }
  default {
    Write-NextSteps -Recommended "Use `$canon-status to inspect the current Canon run."
  }
}

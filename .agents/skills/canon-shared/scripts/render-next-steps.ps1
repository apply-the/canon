param(
  [string]$Profile = "",
  [string]$RunId = "",
  [string]$Target = "",
  [string]$PrimaryArtifactPath = ""
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
    Write-NextSteps -Recommended "None. Review the returned Canon summary first for run $RunId." -PossibleActions @(
      "Use `$canon-status for run $RunId only if you need to refresh the run state.",
      "Use `$canon-inspect-invocations for request-level decisions.",
      "Use `$canon-inspect-evidence only if you need evidence lineage."
    )
  }
  "status-completed" {
    if ($PrimaryArtifactPath) {
      Write-NextSteps -Recommended "None. The run result is already readable for run $RunId." -PossibleActions @(
        "Open the primary artifact at $PrimaryArtifactPath directly when your host supports it.",
        "Use `$canon-inspect-artifacts for the full emitted packet on run $RunId.",
        "Use `$canon-inspect-evidence only if you need lineage or policy rationale for run $RunId."
      )
    } else {
      Write-NextSteps -Recommended "None. The run result is already readable for run $RunId." -PossibleActions @(
        "Use `$canon-inspect-artifacts for the full emitted packet on run $RunId.",
        "Use `$canon-inspect-evidence only if you need lineage or policy rationale for run $RunId."
      )
    }
  }
  "status-gated" {
    Write-NextSteps -Recommended "Use `$canon-inspect-evidence for run $RunId before recording approval." -PossibleActions @(
      "Use `$canon-approve for target $Target on run $RunId after review.",
      "Use `$canon-status after approval, or `$canon-resume if Canon still requires continuation."
    )
  }
  "inspect" {
    Write-NextSteps -Recommended "None. Review the current inspection output directly for run $RunId." -PossibleActions @(
      "Use `$canon-status only if you need to re-check the run state after follow-up work."
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
    Write-NextSteps -Recommended "None. Review the emitted packet directly for run $RunId." -PossibleActions @(
      "Use `$canon-inspect-evidence only if you still need runtime lineage or policy rationale for run $RunId.",
      "Use `$canon-status only after follow-up work changes what you expect from run $RunId."
    )
  }
  default {
    Write-NextSteps -Recommended "Use `$canon-status to inspect the current Canon run."
  }
}

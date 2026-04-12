param(
  [string]$Skill = "",
  [Alias("SkillName")]
  [string]$SkillAlias = "",
  [string]$State = "",
  [Alias("SupportState")]
  [string]$StateAlias = "",
  [string]$Known = "",
  [string]$Missing = "",
  [string]$Nearest = ""
)

if (-not $Skill -and $SkillAlias) { $Skill = $SkillAlias }
if (-not $State -and $StateAlias) { $State = $StateAlias }

Write-Output "Support State: $State"
switch ($State) {
  "modeled-only" { Write-Output "This Canon workflow is modeled, but not runnable end to end yet." }
  "intentionally-limited" { Write-Output "This Canon workflow is intentionally limited in the current release." }
  "experimental" { Write-Output "This Canon workflow is experimental." }
  default { Write-Output "This Canon workflow is available now." }
}

if ($Known) { Write-Output "Known Today: $Known" }
if ($Missing) { Write-Output "Missing: $Missing" }
if ($Nearest) { Write-Output "Nearest Runnable Skill: $Nearest" }
if ($Skill) { Write-Output "Skill: $Skill" }

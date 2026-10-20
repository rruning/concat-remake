# Adds the Built-in shelf's three keys to every locale that lacks them,
# in English, so the shelves show the same words everywhere until they
# are translated. UTF-8 without BOM, CRLF, as the files already are.
$dir = Join-Path $PSScriptRoot 'src\crates\concat\locales'
$enc = New-Object System.Text.UTF8Encoding($false)
$bi = '  "sounds.builtin": "Built-in",'
$hint = '  "sounds.builtinHint": "Sound effects made on this machine, ready with no download. Click one to hear it, press + to add it at the playhead, or drag it onto the timeline.",'
$ui = '  "sounds.catUi": "UI",'
$done = 0
foreach ($f in Get-ChildItem $dir -Filter *.json) {
    if ($f.BaseName -in @('en', 'es')) { continue }
    $lines = [IO.File]::ReadAllLines($f.FullName, $enc)
    if ($lines -match '"sounds\.builtin"') { continue }
    $out = New-Object System.Collections.Generic.List[string]
    foreach ($line in $lines) {
        if ($line -match '"sounds\.catAmbience"') { $out.Add($bi); $out.Add($hint) }
        if ($line -match '"sounds\.catWhoosh"') { $out.Add($ui) }
        $out.Add($line)
    }
    [IO.File]::WriteAllLines($f.FullName, $out, $enc)
    $done++
}
Write-Output "updated: $done"
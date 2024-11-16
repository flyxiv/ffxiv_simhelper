$archeoFiles = Get-ChildItem -Path . -Recurse -File | Where-Object { $_.Name -like "archeo*" }

foreach ($file in $archeoFiles) {
    # 원본 파일의 전체 경로
    $originalPath = $file.FullName
    
    # 새 파일 이름 생성 ("augmented_archeo"로 시작)
    $newFileName = "augmented_" + $file.Name
    
    # 새 파일의 전체 경로
    $newPath = Join-Path -Path $file.DirectoryName -ChildPath $newFileName
    
    # 파일 복사
    Copy-Item -Path $originalPath -Destination $newPath
    
    Write-Host "복사됨: $originalPath -> $newPath"
}

Write-Host "작업 완료!"
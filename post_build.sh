#!/usr/bin/env bash
set -euo pipefail

appName="webimint"
stylePrefix="index"
styleFormat="css"

# Extract build version
indexJsFile=$(find ./dist/.stage -iname "${appName}-*.js")
echo "Extracting build version from file: ${indexJsFile}"
regex="(.*)${appName}-(.*).js"
_src="${indexJsFile}"
while [[ "${_src}" =~ ${regex} ]]; do
    buildVersion="${BASH_REMATCH[2]}"
    _i=${#BASH_REMATCH}
    _src=${_src:_i}
done
if [ -z "${buildVersion}" ]; then
    echo "Could not determine build version!"
    exit 1
fi
echo "Build-Version is: ${buildVersion}"

# Replace placeholder in service-worker.js
serviceWorkerJsFile=$(find ./dist/.stage -iname "service-worker.js")
echo "Replacing {{buildVersion}} placeholder in: ${serviceWorkerJsFile}"
sed -i "s/{{buildVersion}}/${buildVersion}/g" "${serviceWorkerJsFile}"

# Replace placeholder in index.html
indexHtmlFile=$(find ./dist/.stage -iname "index.html")
echo "Replacing {{buildVersion}} placeholder in: ${indexHtmlFile}"
sed -i "s/{{buildVersion}}/${buildVersion}/g" "${indexHtmlFile}"

# Replace placeholder in manifest.json
manifestJsonFile=$(find ./dist/.stage -iname "manifest.json")
echo "Replacing {{buildVersion}} placeholder in: ${manifestJsonFile}"
sed -i "s/{{buildVersion}}/${buildVersion}/g" "${manifestJsonFile}"

# Extract CSS build version
indexJsFile=$(find ./dist/.stage -iname "${stylePrefix}-*.${styleFormat}")
echo "Extracting style build version from file: ${indexJsFile}"
regex="(.*)${stylePrefix}-(.*).${styleFormat}"
_src="${indexJsFile}"
while [[ "${_src}" =~ ${regex} ]]; do
    cssBuildVersion="${BASH_REMATCH[2]}"
    _i=${#BASH_REMATCH}
    _src=${_src:_i}
done
if [ -z "${cssBuildVersion}" ]; then
    echo "Could not determine style build version!"
    exit 1
fi
echo "CSS Build-Version is: ${cssBuildVersion}"

# Replace placeholder in service-worker.js
serviceWorkerJsFile=$(find ./dist/.stage -iname "service-worker.js")
echo "Replacing {{cssBuildVersion}} placeholder in: ${serviceWorkerJsFile}"
sed -i "s/{{cssBuildVersion}}/${cssBuildVersion}/g" "${serviceWorkerJsFile}"

### Required for chrome extension, no inline scripting
echo "Extracting script content from index.html and creating initWebimint.js"
scriptContent=$(sed -n 's|.*<script type=module>\(.*\)</script>.*|\1|p' "${indexHtmlFile}")
echo "${scriptContent}" >./dist/.stage/initWebimint.js
echo "Replacing original script tag in index.html with reference to initWebimint.js"
sed -i 's|<script type=module>[^<]*</script>|<script type="module" src="/initWebimint.js"></script>|' "${indexHtmlFile}"

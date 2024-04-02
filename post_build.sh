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

# ### Required for chrome extension, no inline scripting
echo "Extracting script content from index.html and creating initWebimint.js"
scriptContent=$(sed -n 's|.*<script type=module>\(.*\)</script>.*|\1|p' "${indexHtmlFile}")
if [ -n "${scriptContent}" ]; then
    echo "${scriptContent}" >./dist/.stage/initWebimint.js
    echo "Replacing original script tag in index.html with reference to initWebimint.js"
    sed -i 's|<script type=module>[^<]*</script>|<script type="module" src="/initWebimint.js"></script>|' "${indexHtmlFile}"
else # using trunk serve, multiline script tags, have to extract line by line
    echo "Using trunk serve, multiline script tags, have to extract line by line"
    echo "Extracting script content from index.html and creating separate .js files"
    # Directory where the new JS files will be stored
    JS_DIR="./dist/.stage/js"
    mkdir -p "$JS_DIR"
    # Counter to name the extracted JS files uniquely
    COUNTER=1
    # Temporary file to hold the modified HTML content
    TMP_HTML=$(mktemp)
    # Initialize SCRIPT_OPEN to 0 before the loop
    SCRIPT_OPEN=0
    # Read the index.html file line by line
    while IFS= read -r line || [[ -n "$line" ]]; do
        if [[ $line =~ \<script.*\>\</script\> ]]; then
            # Inline script tag with no content, just copy the line
            echo "$line" >>"$TMP_HTML"
        elif [[ $line =~ \<script.*\>(.*) ]]; then
            # Opening script tag with potential inline content
            SCRIPT_OPEN=1
            # Capture any content on the same line as the opening script tag
            SCRIPT_CONTENT="${BASH_REMATCH[1]}"
            if [[ $SCRIPT_CONTENT ]]; then
                # If there's inline content right after the script tag, add a newline to start accumulating correctly
                SCRIPT_CONTENT+=$'\n'
            fi
        elif [[ $line =~ \</script\> ]]; then
            # Closing script tag, write content to a new JS file
            SCRIPT_FILE="$JS_DIR/extracted_$COUNTER.js"
            echo "$SCRIPT_CONTENT" >"$SCRIPT_FILE"
            # Replace the script tag in HTML with a reference to the new JS file
            echo "<script type=\"module\" src=\"/js/extracted_$COUNTER.js\"></script>" >>"$TMP_HTML"
            COUNTER=$((COUNTER + 1))
            SCRIPT_OPEN=0
            SCRIPT_CONTENT="" # Reset SCRIPT_CONTENT for the next script
        elif [[ $SCRIPT_OPEN -eq 1 ]]; then
            # Inside a script tag, accumulate the content
            SCRIPT_CONTENT+="$line"$'\n'
        else
            # Outside script tags, just copy the line
            echo "$line" >>"$TMP_HTML"
        fi
    done <"${indexHtmlFile}"
    # Replace the original HTML file with the modified one
    mv "$TMP_HTML" "${indexHtmlFile}"
    # Clean up
    rm -f "$TMP_HTML"
    # Replace placeholder trunk address in JavaScript files
    echo "Replacing {{__TRUNK_ADDRESS__}} placeholder in extracted JavaScript files"
    jsFiles=$(find ./dist/.stage/js -iname "*.js")
    TRUNK_ADDRESS=127.0.0.1:8080
    for file in $jsFiles; do
        sed -i "s/{{__TRUNK_ADDRESS__}}/${TRUNK_ADDRESS}/g" "$file"
        echo "Replaced in: $file"
    done
fi

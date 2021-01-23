#!/bin/bash
if (echo ${GITHUB_REF} | egrep -q '^refs/tags/v[0-9]+.*'); then
    CREATE_RELEASE=true
else
    CREATE_RELEASE=false
fi

if ${CREATE_RELEASE} == "true"; then
    ARTIFACT_SUFFIX="-${GITHUB_REF##*/}"
else
    ARTIFACT_SUFFIX=
fi

case $1 in
"CREATE_RELEASE")
    echo "CREATE_RELEASE=${CREATE_RELEASE}"
    ;;
"ARTIFACT_SUFFIX")
    echo "ARTIFACT_SUFFIX=${ARTIFACT_SUFFIX}"
    ;;
esac
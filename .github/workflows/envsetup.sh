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



echo "CREATE_RELEASE=${CREATE_RELEASE}" >> ${GITHUB_ENV}
echo "ARTIFACT_SUFFIX=${ARTIFACT_SUFFIX}" >> ${GITHUB_ENV}
if [ -n "$(git diff --name-only HEAD^ HEAD | grep ^src)" ] 
then 
    echo "source files have been changed"
    if [ -n "$(git diff --name-only HEAD^ HEAD | grep ^tests)" ] 
    then
        echo "and tests have been updated"
    else 
        echo "but tests have not been updated" 
    fi
fi
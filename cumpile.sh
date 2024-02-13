PROJECT_NAME="femscript"
PY_VERSION="3.12"

python$PY_VERSION -m maturin build -i python$PY_VERSION "$@"
pip$PY_VERSION install --force-reinstall target/wheels/*.whl

(echo; echo; cat femscript.py) >> $(pip$PY_VERSION show $PROJECT_NAME | awk '/Location:/ {print $2}')/$PROJECT_NAME/__init__.py
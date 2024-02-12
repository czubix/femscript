PROJECT_NAME="femscript"

python3.12 -m maturin build -i python3.12 "$@"
pip3.12 install --force-reinstall target/wheels/*.whl

(echo; echo; cat femscript.py) >> `pip3.12 show $PROJECT_NAME | awk '/Location:/ {print $2}'`/$PROJECT_NAME/__init__.py
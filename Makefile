PYTHON_VERSION := python3.10
PIP_VERSION := pip3.10

build:
	$(PYTHON_VERSION) -m maturin build -i $(PYTHON_VERSION) $(EXTRA_ARGS)

install:
	$(PIP_VERSION) install --force-reinstall target/wheels/*.whl
	(echo; echo; cat femscript.py) >> $$($(PIP_VERSION) show femscript | awk '/Location:/ {print $$2}')/femscript/__init__.py

clean:
	rm target/wheels/*.whl

%:
	@:
# Structural System
Self-organizing concept-learning framework.

## Build
PYO3_USE_ABI3_FORWARD_COMPATIBILITY=1 maturin build --release

## Install
pip install target/wheels/*.whl --force-reinstall --no-deps

## Usage
from structural_system import PyStructuralSystem
s = PyStructuralSystem(dim=4)
s.step(batch=[[...], [...]])

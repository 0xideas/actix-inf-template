import numpy as np
from sklearn.tree import DecisionTreeClassifier
from skl2onnx import to_onnx
import os

np.random.seed(101)
data = np.random.randn(1000, 3).astype(np.float32)
y = (data.sum(1) / 2).astype(int)
y = y - np.min(y)
print(sorted(np.unique(y)))

#black_op = ["ZipMap"]
black_op = []
clf = DecisionTreeClassifier()
clf.fit(data, y)
onx = to_onnx(clf, data[:1, :], target_opset=12, black_op=black_op)


with open("model/model_sklearn.onnx", "wb") as f:
    f.write(onx.SerializeToString())

os.system(
    "python -m onnxruntime.tools.make_dynamic_shape_fixed --input_name X --input_shape 1,3  model/model.onnx model/model.onnx"
)

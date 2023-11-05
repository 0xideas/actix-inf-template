import numpy as np
import torch
import torch.nn as nn

np.random.seed(101)
data = np.random.randn(1000, 3).astype(np.float32)
y = (data.sum(1) / 2).astype(int)
y = y - np.min(y)
print(sorted(np.unique(y)))


data = torch.from_numpy(data)
y = torch.from_numpy(y)

model = nn.Sequential(
    nn.Linear(3, 5),
    nn.ReLU(),
    nn.Linear(5, 10000),
    nn.ReLU(),
    nn.Linear(10000, 10000),
    nn.ReLU(),
    nn.Linear(10000, 10000),
    nn.ReLU(),
    nn.Linear(10000, 10000),
    nn.ReLU(),
    nn.Linear(10000, 10000),
    nn.ReLU(),
    nn.Linear(10000, 5),
    nn.ReLU(),
    nn.Linear(5, 6),
)
print(model)
loss_fn = nn.CrossEntropyLoss()
optimizer = torch.optim.Adam(model.parameters(), lr=0.001)
for n in range(10):
    print(n)
    y_pred = model(data)
    loss = loss_fn(y_pred, y)
    optimizer.zero_grad()
    loss.backward()
    optimizer.step()

with open("model/large_model.onnx", "wb") as f:
    torch.onnx.export(model, torch.randn(1, 3), f)

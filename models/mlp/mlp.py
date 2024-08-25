#!/usr/bin/env python
# coding: utf-8

# # o1js Benchmark
# 
# This notebook benchmarks an MLP model implemented in PyTorch, which mirrors the structure of an MLP model defined using o1js in TypeScript. The model is then exported to ONNX format.

# In[14]:


# Install necessary packages
get_ipython().system('pip install torch numpy onnx ezkl')


# In[17]:


# Import necessary libraries
import torch
import torch.nn as nn
import numpy as np
import onnx
import os
import json
import ezkl


# In[167]:


exp_num = 6


# In[168]:


get_ipython().system(' mkdir -p mlp{exp_num}')


# In[169]:


model_path = os.path.join(f'mlp{exp_num}/mlp.onnx')
compiled_model_path = os.path.join(f'mlp{exp_num}/model.compiled')
pk_path = os.path.join(f'mlp{exp_num}/pk.key')
vk_path = os.path.join(f'mlp{exp_num}/test.vk')
settings_path = os.path.join(f'mlp{exp_num}/settings.json')

witness_path = os.path.join(f'mlp{exp_num}/witness.json')
data_path = os.path.join('input.json')
onnx_path =  os.path.join(f"mlp{exp_num}/mlp.onnx")


# ## Define the MLP Model

# In[170]:


# Define the MLP class with the same structure as the o1js MLP
class MLP(nn.Module):
    def __init__(self, depth):
        super(MLP, self).__init__()
        self.layers = nn.ModuleList([nn.Linear(5, 5) for _ in range(depth)])
        self.output = nn.Linear(5, 1)
        self.relu = nn.ReLU()
        for m in self.modules():
            if isinstance(m, nn.Linear):
                torch.nn.init.zeros_(m.weight.data)

    def forward(self, x):
        for layer in self.layers:
            x = self.relu(layer(x))
        x = self.output(x)
        return x


# ## Initialize the Model and Set Parameters

# In[171]:


# Initialize the model
depth = 2**exp_num
model = MLP(depth=depth)

# Manually set the weights and biases to match the o1js example
# with torch.no_grad():
#     # `hidden1` expects a [1, 5] weight matrix for a [5] input vector
#     model.hidden1.weight = nn.Parameter(torch.tensor([[2.0, 4.0, 3.0, 1.0, 5.0]]))  # shape [1, 5]
#     model.hidden1.bias = nn.Parameter(torch.tensor([3.0]))  # shape [1]

#     # `hidden2` expects a [1, 5] weight matrix for a [5] input vector (which is repeated 5 times)
#     model.hidden2.weight = nn.Parameter(torch.tensor([[3.0, 1.0, 4.0, 2.0, 6.0]]))  # shape [1, 5]
#     model.hidden2.bias = nn.Parameter(torch.tensor([2.0]))  # shape [1]

#     # `output` expects a [1, 1] weight matrix for a [1] input vector
#     model.output.weight = nn.Parameter(torch.tensor([[1.0]]))  # shape [1, 1]
#     model.output.bias = nn.Parameter(torch.tensor([5.0]))  # shape [1]


# ## Perform Forward Pass

# In[172]:


# Create input data (same as in o1js example)
# read in ./input_json
data = json.load(open("input.json", 'r'))

# convert to torch tensor
input_data = torch.tensor(data['input_data'], requires_grad=True)

# Perform forward pass through the network
output = model(input_data)
print("Model Output:", output)


# ## Export the Model to ONNX Format

# In[173]:


# Export the model to ONNX format
torch.onnx.export(
    model,                          # Model being run
    input_data,                     # Model input (or a tuple for multiple inputs)
    onnx_path,                      # Where to save the model (can be a file or file-like object)
    export_params=True,             # Store the trained parameter weights inside the model file
    opset_version=10,               # The ONNX version to export the model to
    do_constant_folding=True,       # Whether to execute constant folding for optimization
    input_names=['input'],          # The model's input names
    output_names=['output'],        # The model's output names
    dynamic_axes={'input': {0: 'batch_size'}, 'output': {0: 'batch_size'}}  # Variable length axes
)

print(f"Model exported to {onnx_path}")


# ## Validate the ONNX Model

# In[174]:


# Load and check the ONNX model
onnx_model = onnx.load(onnx_path)
onnx.checker.check_model(onnx_model)
print('ONNX model is valid')


# In[175]:


# TODO: Dictionary outputs
res = ezkl.gen_settings(model_path, settings_path)
assert res == True


# In[176]:


# res = ezkl.calibrate_settings(data_path, model_path, settings_path, "resources")
# assert res == True


# In[177]:


res = ezkl.compile_circuit(model_path, compiled_model_path, settings_path)
assert res == True


# In[178]:


# srs path
res = ezkl.get_srs(settings_path)


# In[179]:


# now generate the witness file 

res = await ezkl.gen_witness(data_path, compiled_model_path, witness_path)
assert os.path.isfile(witness_path)


# In[180]:


# HERE WE SETUP THE CIRCUIT PARAMS
# WE GOT KEYS
# WE GOT CIRCUIT PARAMETERS
# EVERYTHING ANYONE HAS EVER NEEDED FOR ZK

res = ezkl.setup(
        compiled_model_path,
        vk_path,
        pk_path,
        
    )

assert res == True
assert os.path.isfile(vk_path)
assert os.path.isfile(pk_path)
assert os.path.isfile(settings_path)


# In[ ]:





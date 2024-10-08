# Layers Action

Layers allow you to add more functionality within a smaller footprint.  
You can think of them like the Shift key: `one key can define how other keys behave`.  
However, unlike Shift, there are multiple ways to enable layers.  
By utilizing layers, orbit provides a flexible and powerful way to enhance your keyboard experience.

Benefits of Using Layers

- **Efficient Space Management**  
  Layers allow for more complex configurations without increasing the physical number of keys.
- **Customization**  
  Create personalized key mappings for different applications or tasks by using various layers.


## Usage

| Method | Code | Description |
| ------ | ------------- | ----------- |
| Momentary Layer | `ml(layer, [modifier_mask])` | Enables the layer while the key is held down |
| Toggle Layer | `tl(layer, [modifier_mask])` | Switches to the layer when pressed. <br> Switches to the previous layer when pressed again |
| Set Layer | `sl(layer, [modifier_mask])` | Switches to the layer when pressed |
| Set Default Layer | `sbl(layer, [modifier_mask])` | Sets the [base layer](#base-layer). <br> This might be used to switch from QWERTY to DVORAK layout etc.  |

## Legend

#### Modifier Mask
This is an [optional](/actions#optional-parameters) parameter.  
You can pass a `modifier mask` that has to be present for the layer action to execute.  
A list of masks can be found [here](/actions/modifiers#masks).  



### Base layer
This layer is the always-active layer that other layers stack on top of.  
In other words, it defines how your basic keys work, without any other layering.  
By default this layer has the number `0`.

## Examples

The following examples only have 4 keys defined for ease of demonstration.

Change layer on press:
```orbit
# when `key 1` is pressed down, it switches to `layer 1`
layer 0 # [!code focus]
_______________________________   # [!code focus]
press | ml(1) q     w     e       # [!code focus]

layer 1
_______________________________
press | ---  1     2     3     

```

Change layer on hold:
```orbit
# normally `key 1` is "esc"
# when `key 1` is held, it switches to `layer 1`
layer 0 # [!code focus]
_______________________________   # [!code focus]
press | esc   q     w     e       # [!code focus]
hold  | ml(1) q     w     e       # [!code focus]

layer 1
_______________________________
press | ---  1     2     3     

```


Change layer with shift:
```orbit
# normally `key 1` is "esc"
# when `key 1` is held AND left shift is held, it toggles to `layer 1`
# this means in `layer 1` we can use `key 1` one again, since it doesnt have to be held anymore
layer 0 # [!code focus]
____________________________________  # [!code focus]
press | esc       q     w     lsft    # [!code focus]
hold  | tl(1, ls) q     w     e       # [!code focus]

layer 1
____________________________________
press | esc       1     2     3     

```








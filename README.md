# Yew Image Drop

A simple sample application using [Yew](https://yew.rs/) and the `use_drop` hook from [yew-hooks](https://docs.rs/yew-hooks/latest/yew_hooks/index.html) and [yewdux](https://github.com/intendednull/yewdux) for state management.

You can drop images onto the page and drag and resize the images afterwards. Every image gets a resize handle for every resize direction, eight in total.

- resize image &rarr; drag any handle
- resize image while keeping the ratio &rarr; drag any handle + `ctrl` key
- remove image &rarr; double click on an image or mouse down + `delete` key
- bring to front &rarr; mouse down + `plus` key
- bring to back &rarr; mouse down + `minus` key
- create a pattern &rarr; hold `shift` key while dragging a resize handler

To create a pattern:

1. add an image to the stage
2. resize the image to your liking; this is size and shape of the image that will be repeated
3. resize the image while holding the `shift` key to create a surface with a pattern

To adjust a pattern simply resize the image again.

## How it works

The following event listeners are registered to the document:

- pointer move
- pointer up
- key down
- key up

Pointer down event listeners are registered to the images and their handles.

As soon as you drop an image onto the page an `ImageData` struct is added to the `images` vector in the store.

#### Dragging an image

When you click on an image its index in the `Images` vector will be stored in the store as `active_image_index`. Also the 'anchor' is stored in the struct of the `ImageData`; this is the position of the pointer down event relative to the image.

When a value is set for `active_image_index` the pointer move coordinates will be stored in the `ImageData` struct which triggers a rerender and thus move the image around.

#### Resizing an image

When you click on a handle its id will be stored in the store as `active_handle_id`. Also `active_image_index` will be set to the index of image that the handle belongs to.

Now the pointer move coordinates will be forwarded to the active handle which causes the handle to recalculate its position and the size of the `ImageContainer` component.

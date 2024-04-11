# Yew Image Drop

A simple sample application using [Yew](https://yew.rs/) and the `use_drop` hook from [yew-hooks](https://docs.rs/yew-hooks/latest/yew_hooks/index.html) and [yewdux](https://github.com/intendednull/yewdux) for state management.

You can drop images onto the page and drag and resize the images afterwards. Every image gets a resize handle for every resize direction, eight in total. Holding down the `ctrl` key while dragging a resize handle will keep the image in its original ratio. You can remove an image by double-clicking. To bring an image to the front click and hold your mouse on an image and press `z` on your keyboard. To mask or show a certain part of the image hold down the `shift` key while dragging a resize handle.

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

# Yew Image Drop

A simple sample application using [Yew](https://yew.rs/), the `use_drop` hook from [yew-hooks](https://docs.rs/yew-hooks/latest/yew_hooks/index.html) and [yewdux](https://github.com/intendednull/yewdux) for state management.

You can drop images onto the page and reposition and resize the images afterwards. Every image gets a resize handle for every resize direction, eight in total. You can also create background patterns.

<b>&#8614;</b> [Live example](https://tweedegolf.github.io/yew-image-drop/) <b>&mapstoleft;</b>

- resize image &rarr; drag any handle
- resize image while keeping the ratio &rarr; drag any handle + `ctrl` key
- remove image &rarr; double click on an image or mouse down + `delete` key
- increase z-index (bring to front) &rarr; mouse down + `plus` key
- decrease z-index (bring to back) &rarr; mouse down + `minus` key
- create a pattern &rarr; hold `shift` key while dragging a resize handler

To create a pattern:

1. add an image to the stage
2. resize the image to your liking; this is size and shape of the image that will be repeated
3. resize the image while holding the `shift` key to create a surface with a pattern

To adjust a pattern simply resize the image again. If you want to resize the size of the surface that displays your pattern you have to press the `shift` while dragging.

Note that to bring an image to front or to back you sometimes have to press the `plus` or `minus` key multiple times because the z-index gets increased or decreased one level after at the time every key press.

See the [videos](#videos) below <b>&mapstodown;</b>

## How it works

The following event listeners are registered to the document:

- pointer move
- pointer up
- key down
- key up

Pointer down event listeners are registered to the images and their handles.

As soon as you drop an image onto the page an `ImageData` struct is added to the `images` vector in the store.

#### Dragging an image

When you click on an image its index in the `Images` vector will be stored in the store as `active_image_index`. Also the 'anchor' is stored in the struct of the `ImageData`; this is the position of the pointer down event relative to the position of the image.

When a value is set for `active_image_index` the pointer move coordinates will be stored in the `ImageData` struct which triggers a rerender and thus moves the image around.

#### Resizing an image

When you click on a handle its id will be stored in the store as `active_handle_id`. Also `active_image_index` will be set to the index of image that the handle belongs to.

Now the pointer move coordinates will be forwarded to the active handle which causes the handle to recalculate its position and the size of the `ImageContainer` component.

### Videos

#### Add image(s) using drag and drop

[Add image drag and drop](https://github.com/tweedegolf/yew-image-drop/assets/299669/7d6409a6-958c-4008-925f-d9bbf77e372b)

#### Add image(s) using the file menu

[Add image file menu](https://github.com/tweedegolf/yew-image-drop/assets/299669/78378fb0-a2fa-4422-8b78-2a14c77681e3)

#### Resize image with and without keeping the original ratio by holding down the `ctrl` key

[Resize image](https://github.com/tweedegolf/yew-image-drop/assets/299669/fa0e3e9f-e1a5-4d85-8c53-ee7fe31c72d2)

#### Create pattern

[Create pattern](https://github.com/tweedegolf/yew-image-drop/assets/299669/be766c8a-c576-4b65-85e3-eaa265637a72)

#### Change z-index of image by mouse down + `plus` or `minus` key

[Change z-index](https://github.com/tweedegolf/yew-image-drop/assets/299669/5200aad9-3e08-4c1b-839a-a3b01e5b6c42)

#### Resize pattern

[Resize pattern](https://github.com/tweedegolf/yew-image-drop/assets/299669/a61d1293-1b4b-4a8b-b560-b54c550ad237)

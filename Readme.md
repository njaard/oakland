Oakland is a GUI library written in pure Rust and XCB.

It does its own drawing and event management.

It's not really usable.

Here's a program which just shows a window:


```
	let g = Rc::new(oakland::Graphical::new());
	let main = g.put(oakland::MainWindow::new("My Main Window"));
	g.exec();
```

When you `put` a widget onto another, it returns the widget
in an `Rc`.

You can post events to the GUI thread by by calling
`Graphical::channel(handler)`, which simply creates a channel
which activates the function `handler` on the event loop.

Only push buttons, labels, and tabbars are supported right now,
and they don't look great.

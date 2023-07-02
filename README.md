# spotify_dev_patcher

A more unconventional way of enabling developer features in the Spotify desktop client.
Where most techniques involve patching the ``xpui.js`` file, this does _not_ modify any files on disk, and all the changes are completely temporary.
This method should also theoretically be more robust, although my implementation is quite questionable as I do not usually write Rust.

In case this breaks in the future, here's a short write-up on how to find all the relevant addresses:

- Look for the string ``403 Access Denied`` in your disassembler of choice; this string is used in the function which serves the remote debugger webpage, functionality which is locked behind the developer mode flag

- Once you found it, there should be a check above the function that looks something along these lines:
  
  ![](https://i.imgur.com/pZtGE6F.png)

- In our case ``sub_1400B1604`` is the function which checks the dev mode flag, and opening this function will get you the specific byte you have to overwrite.

Although this is not terribly useful, it does open up some pretty cool hidden Spotify debug menus, through which you can enable different experiments and also play around with the internal Esperanto API. (to access this menu, click the three dots in the top-left corner of your client and access the ``Develop`` submenu)

By using the ``--remote-debugging-port`` and ``--remote-allow-origins=*`` flags you can also enable remote debugging.


<div align="center">

  <h1>🦀 crow 🦇</h1>

  <p>
    <strong>crow manages files - mostly .dot files</strong>
  
  _(Pretend the bat is a crow)_
  
  </p>
  &nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;<img src="demo-crow.gif">
</div>

---

#### *Note - this is not currently in a state for other users, if you somehow manage to stumble upon this. It is currently just a personal project*

---

### Workflow:
#### To add a file to crow for management:
- `crow -a YourAlias -s ~/path/to/your/file.conf`
- You can now open this file with `crow -a YourAlias`

This feature is essentially like adding an alias to your .bashrc

It adds alias entries to a file at ~/.config/crow/ named 'crowfile'

Formatted like so: `-<<<>  YourAlias: /path/to/your/alias`

----
#### To pull those files into your "nest":
- `crow -P YourAlias`

This will create the necessary subfolders for the config file at '~/nest' and move the file listed for the alias to the appropriate subfolder.

This is where [gnustow](https://www.gnu.org/software/stow/) 📦 comes into play. Simply `cd ~/nest` and `stow YourAlias`

This should create a symlink where the file was originally located, keeping your configuration intact without having to make a copy in another folder. 

After this, simply `git init` your nest, and manage through github, gitlab, etc.

---
#### Groups!
Groups are for managing a subset of files, rather than one single file. An example might be managing all of your X files (_I want to believe 👽_). Simply set up a group name for the files, in the example we will call it xgroup. 

To establish the group:

- `crow -G xgroup`

This will write the group alias to your crowfile, and will create a sub-directory with the groups name in your nest. 

To pull files into that sub-directory:
- `crow -a filealias -g xgroup`

After the group is set and the files are pulled in, stow it same as any other alias directory:

- `cd ~/nest`
- `stow xgroup`

Another good example use for this would be to establish system config "Profiles" to deploy whenever you feel like trying or creating a new config. Just establish a group directory: `crow -G Profile1`, stow all of your current configs in it with crow, push out the entire directory with stow, and when you feel like resetting to default, just `stow -D Profile1 && stow Profile2`.  

_Experiment, but be careful._

---

### Updates and things to change

_Got group functionality going. Once you set a group with `crow -G GroupAlias` you can now use `crow -a FileAlias -g GroupAlias` to move that file into the group folder._

Also going to be setting up a command that will auto cd you into the group directory, just because. Not mega important, but I can see where I personally would use this. Will update this as I come up with ideas for this.

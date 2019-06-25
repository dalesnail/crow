
<div align="center">

  <h1>🦀 crow 🦇</h1>

  <p>
    <strong>crow manages files - mostly .dot files</strong>
  
  _(Pretend the bat is a crow)_
  
  </p>
  &nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;<img src="demo-crow.gif">
</div>

---

#### *Note - this is in testing currently, so it may be buggy. Also always back up any files managed through crow, even though there should not be any danger to them. I will be writing in a backup feature for the dots at some point, so we don't have to worry about this.*

---

### Workflow:

#### To install
- Download the latest [release](https://github.com/dalesnail/crow/releases)
- Untar the file wherever you would like
- Move or copy the crow binary to your runpath (ex. `cp ./crow /usr/local/bin/`)

#### Init for first time use
- `crow -i`

This will set up the necessary files and directories for crow. It will create a "nest" in your home directory, and will generate the crowfile at `~/.config/crowfile`. This file will hold all of your aliases, as well as your default editor you would like for your files to be opened in. After creating your alias, `crow -a crowfile` will open your crowfile, and you can change what editor you will use. It defaults to Vim currently.

---

#### To add a file to crow for management:
- `crow -a YourAlias -s ~/path/to/your/file.conf`
- You can now open this file with `crow -a YourAlias`

This feature is essentially like adding an alias to your .bashrc

_When you create your first alias, if you have not initiated, your "crowfile" will be generated. _

---

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

Another good example use for this would be to establish system config "Profiles" to deploy whenever you feel like trying or creating a new config. Just establish a group directory: `crow -G Profile1`, stow all of your current configs in it with crow, push out the entire directory with stow, and when you feel like trying another setup, just `stow -D Profile1 && stow Profile2`.  

_Experiment, but be careful._

---

### Updates and things to change

_Got group functionality going. Once you set a group with `crow -G GroupAlias` you can now use `crow -a FileAlias -g GroupAlias` to move that file into the group folder._

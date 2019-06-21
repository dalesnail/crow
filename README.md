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
- After this has completed you can open this file with `crow -a YourAlias`

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

### Updates and things to change

~~_**Regarding the pull feature here**, it currently only pulls one file and makes a directory for that file. Thinking about it, that may not be ideal in some cases, so I am going to write a "pull all" feature into this as well, that will pull all files and directories that exist within the aliasfiles current directory. This should be fine with any number of files since stow will just symlink anything in the folder._~~

Pull all feature was deemed dangerous and not flexible. Moved towards a new idea that has been working well. Added a feature that will establish a Group alias, and create a directory for that alias in your nest. I will be writing a few options to be able to pull files into those group folders rather than one single dir assigned to the file alias. 
This will be better for programs that require more than one config file, but it can also be used to manage a small subset of files for anything really. 
A good example use for this would be to establish system config "Profiles" to deploy whenever you feel like trying or creating a new rice. Just establish a directory `crow -G Profile1`, stow all of your current configs in here with crow, push out the entire directory with stow, and when you feel like resetting to default, just `stow -D Profile1`.  

Also going to be setting up a command that will auto cd you into the group directory, just because. Not mega important, but I can see where I personally would use this. 

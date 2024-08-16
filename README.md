# dcadmin

## What is this?

This is a cross-platform (meaning: Windows, MacOS, Linux) desktop app that currently serves a single purpose: provide a way to obtain reports for DRACOON Server after the deprecation of the old reporting tool.
The app therefore currently provides the following functionality:
- login via OAuth (authorization code) to support all auth methods
- list events / audit log (only DRACOON Server)
- list user permissions (only DRACOON Server)
- list node permissions (only DRACOON Server)
- list users

### What is this really?
This is a desktop app built with [Tauri](https://tauri.app) - it uses Rust on the "backend" and all DRACOON related operations are handled via [dco3](https://github.com/unbekanntes-pferd/dco3). 
The frontend is powered by [SvelteKit](https://kit.svelte.dev).

## Installation

Installers will be provided on the [Releases](https://github.com/unbekanntes-pferd/dcadmin/releases) page and will be available as 
- Windows installer 
- MacOS dmg
- Linux: tbd

## How to use

### Login

In order to log in, enter your DRACOON domain (https:// not needed).
![login](/assets/small/login.png)

You will be prompted to enter an authorization code and will be required to login via browser.
This might be subject to change in the future, but using a custom callback across all platforms has so far not worked as intended.

### Events
In order to view the audit log based on the events, click on the menu entry. You can filter by:
- from date
- to date
- specific action
- specific user

![events filter](/assets/small/events_filter.png)

To clear all filters, click the `Reset` button.

You can download the current selection of events as CSV file - click on the `Download` button to do so.


### Permissions

Permissions are grouped either by users or nodes - this means you can list all permissions either by: 
- user
- node (room)

To do so, click on the relevant menu entry.

#### User permissions

In order to fetch all user permissions, search for a user in the search box and all relevant permissions will appear. 

**❗Important disclaimer:** The old reporting functionality is limited and this tool cannot fix the shortcomings of the endpoints needed to retrieve the info. This means that the usability might be limited for very large instances, specifically if you have many nodes on root and want to browse through node permissions.
In order to avoid these issues, please download all permissions on the user permissions menu and filter locally with the downloaded csv file for specific rooms.

![user permissions](/assets/small/user_permissions.png)

**❗Please note:** Only a subset is displayed in this view. In order to view (e.g. custom) permissions, just hover over the permissions column and a popup will display the permissions.

You can download the individual permissions for a user using the `Download` button.
If you want to retrieve all permissions (for all users / nodes) use the `Download all` button.

#### Node permissions

In order to browser through nodes, please use the `Nodes` menu entry. 
**❗Important disclaimer:** Do not use this if you have thousands of rooms directly on root - this will lead to degraded performance and usability issues. If you know you have a large instance, please download all permissions via `Users` and work locally with a csv file.

![node permissions](/assets/small/nodes.png)

Once you have found the room you are looking for, you can view the permissions by clicking on the permissions icon in the permissions column for a given room.


![node permissions](/assets/small/node_permissions.png)

You can download permissions of a given node by clicking on the `Download` button.



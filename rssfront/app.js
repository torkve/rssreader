function _fd94335e754076f565cc0afec3c40c4537b8a212(){};function _2391b4e0fa677847fb9890ce9418f86a804898d9(){};function _0facbfb76234ea9ddbb12a405727d74a5e6f4eba(){};function _57b4aa201af5c235856e8501b58ca72c0b4987ed(){};function _29a5ac2e2fd3efc4b6e703d460b212529068ad1e(){};/*
    This file is generated and updated by Sencha Cmd. You can edit this file as
    needed for your application, but these edits will have to be merged by
    Sencha Cmd when it performs code generation tasks such as generating new
    models, controllers or views and when running "sencha app upgrade".

    Ideally changes to this file would be limited and most work would be done
    in other places (such as Controllers). If Sencha Cmd cannot merge your
    changes and its generated code, it will produce a "merge conflict" that you
    will need to resolve manually.
*/

Ext.application({
    name: 'rssfront',

    requires: [
        'rssfront.util.Config',
        'Ext.Panel',
        'Ext.data.Store',
        'Ext.dataview.DataView',
    ],

    util: [
        'Config',
    ],
    views: [
        'Main',
        'Posts'
    ],
    stores: [
        'Channels',
        'Posts'
    ],
    models: [
        'Channel',
        'Post'
    ],
    forms: [
        'ChannelAdd'
    ],

    icon: {
        '57': 'resources/icons/Icon.png',
        '72': 'resources/icons/Icon~ipad.png',
        '114': 'resources/icons/Icon@2x.png',
        '144': 'resources/icons/Icon~ipad@2x.png'
    },

    isIconPrecomposed: true,

    startupImage: {
        '320x460': 'resources/startup/320x460.jpg',
        '640x920': 'resources/startup/640x920.png',
        '768x1004': 'resources/startup/768x1004.png',
        '748x1024': 'resources/startup/748x1024.png',
        '1536x2008': 'resources/startup/1536x2008.png',
        '1496x2048': 'resources/startup/1496x2048.png'
    },

    launch: function() {
        // Destroy the #appLoadingIndicator element
        Ext.fly('appLoadingIndicator').destroy();

        // Initialize the main view
        Ext.Viewport.add(Ext.create('rssfront.view.Main'));
    },

    onUpdated: function() {
        Ext.Msg.confirm(
            "Application Update",
            "This application has just successfully been updated to the latest version. Reload now?",
            function(buttonId) {
                if (buttonId === 'yes') {
                    window.location.reload();
                }
            }
        );
    }
});

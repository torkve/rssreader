Ext.define('rssfront.view.Main', {
    extend: 'Ext.navigation.View',
    xtype: 'mainpanel',
    requires: [
        'Ext.navigation.View',
        'rssfront.view.Channels',
        'rssfront.form.ChannelAdd',
    ],
    config: {
        id: 'mainWindow',
        items: [
            {
                title: 'RSS',
                items: [
                    {
                        xtype: 'channelslist'
                    },
                    {
                        xtype: 'toolbar',
                        docked: 'bottom',
                        items: [
                            {
                                xtype: 'button',
                                iconCls: 'add',
                                handler: function() {
                                    Ext.getCmp('mainWindow').push({xtype: 'channeladd'});
                                }
                            }
                        ]
                    }
                ]
            }
        ]
    }
});

Ext.define('rssfront.form.ChannelAdd', {
  extend: 'Ext.form.Panel',
  requires: [
    'Ext.form.FieldSet',
    'Ext.field.Text',
  ],
  xtype: 'channeladd',
  config: {
    fullscreen: true,
    autoDestroy: true,
    items: [
      {
        xtype: 'fieldset',
        items: [
          {
            xtype: 'textfield',
            name: 'url',
            label: 'Feed URL'
          }
        ]
      },
      {
        xtype: 'button',
        docked: 'bottom',
        text: 'Add',
        handler: function() {
          var form = this.getParent();
          Ext.Ajax.request({
            url: rssfront.util.Config.baseUrl() + '/add',
            timeout: 10000,
            method: 'POST',
            params: {
              url: form.getValues().url
            },
            success: function(response) {
              var main = Ext.getCmp('mainWindow');
              console.log("added with success");
              Ext.getStore('Channels').load();
              Ext.getCmp('channelsList').refresh();
              main.pop(main.getInnerItems().length - 1);
            }
          });
        }
      }
    ]
  }
})

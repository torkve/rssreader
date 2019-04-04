Ext.define('rssfront.view.Channels', {
  extend: 'Ext.List',
  xtype: 'channelslist',
  requires: [
    'rssfront.store.Channels',
    'Ext.Toolbar',
    'Ext.plugin.PullRefresh',
  ],
  config: {
    title: 'Channels',
    grouped: false,
    id: 'channelsList',
    itemTpl: '<b>{title}</b><br/><small>{last_updated}</small>',
    disableSelection: true,
    emptyText: 'No channels yet',
    store: 'Channels',
    width: '100%',
    height: '100%',
    plugins: [
      {
        xclass: 'Ext.plugin.PullRefresh',
        pullText: 'Pull to refresh...'
      }
    ],
    listeners: {
      itemtap: function(list, index, target, record) {
        var store = Ext.getStore('Posts');
        store.feedUrl = record.data.url;
        store.load();
        Ext.getCmp('mainWindow').push({
          xtype: 'postslist',
        })
      },
      itemswipe: function(list, index, target, record) {
        Ext.Ajax.request({
          url: rssfront.util.Config.baseUrl() + '/remove',
          timeout: 10000,
          method: "POST",
          params: {
            url: record.data.url
          },
          success: function(response) {
            list.getStore().remove([record]);
          }
        })
      }
    },
  }
});

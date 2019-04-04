Ext.define('rssfront.view.Posts', {
  extend: 'Ext.List',
  xtype: 'postslist',
  requires: [
    'rssfront.store.Posts',
    'Ext.plugin.PullRefresh'
  ],
  autoDestroy: true,
  config: {
    title: 'Posts',
    grouped: false,
    itemTpl: '<div class="rsspost rsspost-{unread}"><b>{title}</b><br/><small>{pub_date}</small></div>',
    disableSelection: true,
    emptyText: 'No posts yet',
    store: 'Posts',
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
        Ext.Ajax.request({
          url: rssfront.util.Config.baseUrl() + '/read',
          timeout: 10000,
          method: "POST",
          params: {
            url: list.getStore().feedUrl,
            guid: record.data.guid
          },
          success: function(response) {
            record.data.unread = false;
            list.refresh();
            Ext.getCmp('mainWindow').push({
              xtype: 'panel',
              title: 'Post',
              styleHtmlContent: true,
              scrollable: 'vertical',
              html: record.formatPost(),
              autoDestroy: true
            });
          }
        });
      }
    }
  }
});


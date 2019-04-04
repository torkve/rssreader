Ext.define('rssfront.store.Posts', {
  extend: 'Ext.data.Store',
  config: {
    model: 'rssfront.model.Post',
    sorters: {
      property: 'pub_date',
      direction: 'DESC'
    },
    autoLoad: false,
    feedUrl: null,
    scope: this,
    listeners: {
      beforeload: function(store) {
        store.setProxy({
          type: 'ajax',
          url: rssfront.util.Config.baseUrl() + '/get?url=' + encodeURIComponent(store.feedUrl),
          enablePagingParams: false,
          reader: {
            type: 'json',
            rootProperty: 'items'
          }
        });
      }
    },
  }
});


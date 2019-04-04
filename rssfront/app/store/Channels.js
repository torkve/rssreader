Ext.define('rssfront.store.Channels', {
  extend: 'Ext.data.Store',
  config: {
    model: 'rssfront.model.Channel',
    sorters: {
      property: 'last_updated',
      direction: 'DESC'
    },
    autoLoad: true,
    scope: this,
    proxy: {
      type: 'ajax',
      url: rssfront.util.Config.baseUrl() + '/list',
      enablePagingParams: false,
      reader: {
        type: 'json'
      }
    }
  }
});

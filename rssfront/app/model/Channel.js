Ext.define('rssfront.model.Channel', {
  extend: 'Ext.data.Model',
  config: {
    idProperty: 'url',
    fields: [
      'title', 'url',
      {
        name: 'last_updated',
        type: 'Date',
        sortDir: 'DESC',
      }
    ]
  }
});

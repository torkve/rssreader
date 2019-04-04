Ext.define('rssfront.model.Post', {
  extend: 'Ext.data.Model',
  config: {
    idProperty: 'guid',
    fields: [
      'title', 'link', 'content', 'guid',
      {
        name: 'pub_date',
        type: 'Date',
        sortDir: 'DESC',
      },
      {
        name: 'unread',
        type: 'bool'
      }
    ]
  },
  formatPost: function() {
    var data = this.data
    var header = '<b>' + data.title + '</b>';
    if (data.link) header = '<a href="' + encodeURI(data.link) + '">' + header + '</a>';
    var date = '<small>' + data.pub_date + '</small>';

    return [header, date, '', data.content].join('<br/>');
  }
});

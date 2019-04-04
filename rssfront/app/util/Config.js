Ext.define('rssfront.util.Config', {
    singleton: true,

    config: {
        basePort: 8080,
        basePath: '',
    },


    constructor: function(config) {
        this.initConfig(config);
        this.callParent([config]);
        this.baseUrl = function() {
            var port =  '';
            if (window.location.protocol == 'https:' && this.config.basePort != 443) port = ':' + this.config.basePort;
            if (window.location.protocol == 'http:' && this.config.basePort != 80) port = ':' + this.config.basePort;

            return '//' + window.location.hostname + port + this.config.basePath;
        };
    }
});

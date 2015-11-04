class ApiService {
    constructor($http: any, $rootScope: any) {
        this.$http = $http;
        this.$rootScope = $rootScope;
        this.config = $rootScope.config;
    }

    $http: any;
    $rootScope: any;
    config: Config;

    get(url: string, params: any, eventType: EventType) {
        this.$http({
            method: 'GET',
            url: url,
            params: params,
            headers: { 'Authorization': 'Token token=xxxxYYYYZzzz' }
        }).then((result) => {
            this.$rootScope.$broadcast(eventType, {
                data: result.data
            });
        });
    }

    getSliceScript() {
        var url = 'http://localhost:52046/api/slices/slicescript';
        this.get(url, {}, EventType.SliceScript);
    }
}
# Kubernetes StatefulSet 节点分组部署指南

## 需求说明

在 K8s 集群中使用 StatefulSet 启动 4 个 Pod，要求：
- 将 4 个 Pod 分成两组
- 第一组（2个Pod）部署在节点1
- 第二组（2个Pod）部署在节点2

---

## 解决方案

### 方法 1：使用 topologySpreadConstraints（推荐）

```yaml
apiVersion: apps/v1
kind: StatefulSet
metadata:
  name: my-statefulset
spec:
  serviceName: my-service
  replicas: 4
  selector:
    matchLabels:
      app: my-app
  template:
    metadata:
      labels:
        app: my-app
    spec:
      topologySpreadConstraints:
      - maxSkew: 1
        topologyKey: kubernetes.io/hostname
        whenUnsatisfiable: DoNotSchedule
        labelSelector:
          matchLabels:
            app: my-app
        # 这会确保 Pod 在节点间均匀分布（每个节点 2 个 Pod）
      affinity:
        nodeAffinity:
          requiredDuringSchedulingIgnoredDuringExecution:
            nodeSelectorTerms:
            - matchExpressions:
              - key: node-group
                operator: In
                values:
                - group1
                - group2
      containers:
      - name: my-container
        image: your-image:tag
        ports:
        - containerPort: 8080
```

**前提条件**：需要给节点打标签

```bash
# 标记节点1为 group1
kubectl label nodes node1 node-group=group1

# 标记节点2为 group2
kubectl label nodes node2 node-group=group2
```

---

### 方法 2：使用 podAntiAffinity 强制分散

```yaml
apiVersion: apps/v1
kind: StatefulSet
metadata:
  name: my-statefulset
spec:
  serviceName: my-service
  replicas: 4
  selector:
    matchLabels:
      app: my-app
  template:
    metadata:
      labels:
        app: my-app
    spec:
      affinity:
        # 节点亲和性：只能调度到指定的两个节点
        nodeAffinity:
          requiredDuringSchedulingIgnoredDuringExecution:
            nodeSelectorTerms:
            - matchExpressions:
              - key: kubernetes.io/hostname
                operator: In
                values:
                - node1  # 替换为实际节点名
                - node2  # 替换为实际节点名
        # Pod 反亲和性：确保每个节点不超过 2 个 Pod
        podAntiAffinity:
          preferredDuringSchedulingIgnoredDuringExecution:
          - weight: 100
            podAffinityTerm:
              labelSelector:
                matchLabels:
                  app: my-app
              topologyKey: kubernetes.io/hostname
      containers:
      - name: my-container
        image: your-image:tag
        ports:
        - containerPort: 8080
```

---

### 方法 3：结合使用（最精确控制）⭐

```yaml
apiVersion: apps/v1
kind: StatefulSet
metadata:
  name: my-statefulset
spec:
  serviceName: my-service
  replicas: 4
  selector:
    matchLabels:
      app: my-app
  template:
    metadata:
      labels:
        app: my-app
    spec:
      topologySpreadConstraints:
      - maxSkew: 0  # 0 表示必须完全均匀分布
        topologyKey: kubernetes.io/hostname
        whenUnsatisfiable: DoNotSchedule
        labelSelector:
          matchLabels:
            app: my-app
      affinity:
        nodeAffinity:
          requiredDuringSchedulingIgnoredDuringExecution:
            nodeSelectorTerms:
            - matchExpressions:
              - key: kubernetes.io/hostname
                operator: In
                values:
                - node1
                - node2
      containers:
      - name: my-container
        image: your-image:tag
        ports:
        - containerPort: 8080
        volumeMounts:
        - name: data
          mountPath: /data
  volumeClaimTemplates:
  - metadata:
      name: data
    spec:
      accessModes: [ "ReadWriteOnce" ]
      resources:
        requests:
          storage: 10Gi
```

---

## 验证部署

部署后，使用以下命令验证 Pod 分布：

```bash
# 查看 Pod 及其所在节点
kubectl get pods -l app=my-app -o wide

# 按节点统计 Pod 数量
kubectl get pods -l app=my-app -o wide | awk 'NR>1 {print $7}' | sort | uniq -c
```

---

## 关键参数说明

### topologySpreadConstraints

控制 Pod 在拓扑域间的分布：

- **maxSkew**: 允许的最大偏差值
  - `maxSkew: 0` - 必须完全均匀（每个节点 2 个 Pod）
  - `maxSkew: 1` - 允许最多相差 1 个 Pod

- **topologyKey**: 拓扑域的键
  - `kubernetes.io/hostname` - 按节点分布
  - `topology.kubernetes.io/zone` - 按可用区分布

- **whenUnsatisfiable**: 不满足条件时的行为
  - `DoNotSchedule` - 不满足条件就不调度（强制）
  - `ScheduleAnyway` - 尽量满足但不强制

### nodeAffinity

限制 Pod 只能调度到特定节点：

- **requiredDuringSchedulingIgnoredDuringExecution**: 硬性要求，必须满足
- **preferredDuringSchedulingIgnoredDuringExecution**: 软性要求，尽量满足

### podAntiAffinity

防止 Pod 过于集中部署：

- **requiredDuringSchedulingIgnoredDuringExecution**: 强制反亲和
- **preferredDuringSchedulingIgnoredDuringExecution**: 偏好反亲和

---

## 实际应用步骤

### 1. 查看现有节点

```bash
kubectl get nodes
```

### 2. 给节点打标签（可选）

```bash
kubectl label nodes <node1-name> node-group=group1
kubectl label nodes <node2-name> node-group=group2
```

### 3. 创建 Service（StatefulSet 必需）

```yaml
apiVersion: v1
kind: Service
metadata:
  name: my-service
spec:
  clusterIP: None  # Headless Service
  selector:
    app: my-app
  ports:
  - port: 8080
    targetPort: 8080
```

### 4. 应用配置

```bash
# 创建 Service
kubectl apply -f service.yaml

# 创建 StatefulSet
kubectl apply -f statefulset.yaml
```

### 5. 验证部署结果

```bash
# 查看 StatefulSet 状态
kubectl get statefulset my-statefulset

# 查看 Pod 分布
kubectl get pods -o wide -l app=my-app

# 详细查看 Pod 调度信息
kubectl describe pod my-statefulset-0
```

---

## 故障排查

### Pod 一直处于 Pending 状态

```bash
# 查看 Pod 详情
kubectl describe pod <pod-name>

# 常见原因：
# 1. 节点资源不足
# 2. 节点标签不匹配
# 3. 拓扑约束无法满足
# 4. PVC 无法绑定
```

### Pod 没有均匀分布

```bash
# 检查拓扑约束配置
kubectl get statefulset my-statefulset -o yaml | grep -A 10 topologySpreadConstraints

# 检查节点是否可调度
kubectl get nodes

# 检查节点污点
kubectl describe node <node-name> | grep Taints
```

---

## 推荐配置

选择**方法 3**（结合使用）可以获得最精确的控制，确保：

✅ 每个节点恰好有 2 个 Pod  
✅ Pod 只会调度到指定的两个节点  
✅ 分布强制执行，不满足条件不调度  
✅ 支持 StatefulSet 的持久化存储需求  

---

## 注意事项

1. **StatefulSet 的有序性**: Pod 按顺序创建（0, 1, 2, 3），但调度器会根据约束分配到不同节点
2. **持久化存储**: 每个 Pod 绑定独立的 PVC，确保数据持久化
3. **滚动更新**: 更新时需要考虑节点亲和性，可能影响更新顺序
4. **节点维护**: 如果某个节点不可用，该节点上的 Pod 不会自动迁移到其他节点（StatefulSet 特性）
5. **扩缩容**: 扩容时新 Pod 会自动遵循分布约束

---

## 参考资料

- [Kubernetes StatefulSet 官方文档](https://kubernetes.io/docs/concepts/workloads/controllers/statefulset/)
- [Pod Topology Spread Constraints](https://kubernetes.io/docs/concepts/scheduling-eviction/topology-spread-constraints/)
- [Affinity and anti-affinity](https://kubernetes.io/docs/concepts/scheduling-eviction/assign-pod-node/#affinity-and-anti-affinity)

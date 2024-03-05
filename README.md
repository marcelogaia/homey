# Home Assistant System

This repository contains the code for a home assistant system designed to monitor and control smart devices within a home environment. The system is built using Rust for the backend, with plans to extend functionality over time. The project is containerized using Docker and orchestrated with Kubernetes using K3s for deployment on a Raspberry Pi.

## Components

- [ ] **Frontend**: Displays data and controls for the home assistant system.
- [ ] **Rust Backend**: Manages data collection, processing, and API services.
- [ ] **SQLite Database**: Stores historical data for temperature and device events.
- [ ] **Kubernetes (K3s)**: Orchestrates container deployment, scaling, and management.

## Development Setup

- [x] Install Docker and Docker Compose on your local machine.
- [ ] Install kubectl and set up K3s on your Raspberry Pi.
- [x] Clone this repository to your local development machine.

## Deployment with K3s

This project uses K3s for lightweight Kubernetes deployment:

- [ ] Set up K3s on your Raspberry Pi (refer to the K3s documentation).
- [ ] Apply Kubernetes configurations using Kustomize and ArgoCD for continuous deployment.

## Configuration Management with Kustomize

Kustomize is used to manage Kubernetes resources for different environments:

- [ ] Use `kubectl apply -k` to apply configurations for the desired environment.

## Continuous Deployment with ArgoCD

ArgoCD automates the deployment from this repository to the Kubernetes cluster:

- [ ] Set up ArgoCD on your K3s cluster.
- [ ] Connect ArgoCD to this GitHub repository.
- [ ] Monitor deployments through the ArgoCD dashboard.

## Monitoring with Prometheus and Grafana

Monitoring is set up using Prometheus for metrics collection and Grafana for visualization:

- [ ] Apply the Prometheus and Grafana configurations to your K3s cluster.
- [ ] Access Grafana to view dashboards and monitor your system's health.

## Getting Started

- [ ] Detailed instructions on setting up the development environment, deploying with K3s, managing configurations with Kustomize, setting up CD with ArgoCD, and monitoring with Prometheus and Grafana are available in the respective directories.

## Contributing

- Contributions to this project are welcome. Please ensure to follow the coding standards and commit guidelines.
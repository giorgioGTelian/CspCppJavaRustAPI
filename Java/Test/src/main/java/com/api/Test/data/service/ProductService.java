package com.api.Test.data.service;
import java.util.List;
import java.util.Optional;

import org.springframework.stereotype.Service;

import com.api.Test.data.entity.Product;
import com.api.Test.data.repository.ProductRepository;

    @Service
    public class ProductService {
        private final ProductRepository productRepository;

        public ProductService(ProductRepository productRepository) {
            this.productRepository = productRepository;
        }

        public Product saveProduct(Product product) {
            // TODO Auto-generated method stub
            throw new UnsupportedOperationException("Unimplemented method 'saveProduct'");
        }

        public List<Product> fetchAllProducts() {
            // TODO Auto-generated method stub
            throw new UnsupportedOperationException("Unimplemented method 'fetchAllProducts'");
        }

        public Optional<Product> fetchProductById(Long id) {
            // TODO Auto-generated method stub
            throw new UnsupportedOperationException("Unimplemented method 'fetchProductById'");
        }
    }    



